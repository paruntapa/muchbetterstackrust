/*
  Warnings:

  - You are about to drop the column `timeAdded` on the `Website` table. All the data in the column will be lost.
  - You are about to drop the column `userId` on the `Website` table. All the data in the column will be lost.
  - You are about to drop the column `region_Id` on the `website_tick` table. All the data in the column will be lost.
  - You are about to drop the column `website_Id` on the `website_tick` table. All the data in the column will be lost.
  - A unique constraint covering the columns `[username]` on the table `User` will be added. If there are existing duplicate values, this will fail.
  - Added the required column `region_id` to the `website_tick` table without a default value. This is not possible if the table is not empty.
  - Added the required column `website_id` to the `website_tick` table without a default value. This is not possible if the table is not empty.

*/
-- DropForeignKey
ALTER TABLE "website" DROP CONSTRAINT "website_userId_fkey";

-- DropForeignKey
ALTER TABLE "website_tick" DROP CONSTRAINT "website_tick_region_Id_fkey";

-- DropForeignKey
ALTER TABLE "website_tick" DROP CONSTRAINT "website_tick_website_Id_fkey";

-- AlterTable
ALTER TABLE "website" DROP COLUMN "timeAdded",
DROP COLUMN "userId",
ADD COLUMN     "time_added" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
ADD COLUMN     "user_id" TEXT;

-- AlterTable
ALTER TABLE "website_tick" DROP COLUMN "region_Id",
DROP COLUMN "website_Id",
ADD COLUMN     "region_id" TEXT NOT NULL,
ADD COLUMN     "website_id" TEXT NOT NULL;

-- CreateIndex
CREATE UNIQUE INDEX "user_username_key" ON "user"("username");

-- AddForeignKey
ALTER TABLE "website" ADD CONSTRAINT "website_user_id_fkey" FOREIGN KEY ("user_id") REFERENCES "user"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "website_tick" ADD CONSTRAINT "website_tick_website_id_fkey" FOREIGN KEY ("website_id") REFERENCES "website"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "website_tick" ADD CONSTRAINT "website_tick_region_id_fkey" FOREIGN KEY ("region_id") REFERENCES "region"("id") ON DELETE RESTRICT ON UPDATE CASCADE;
